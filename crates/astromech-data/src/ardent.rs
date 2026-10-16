//! Ardent Insight (documented REST API, fed by EDDN): where to buy or sell a commodity near a
//! system, the nearest station with a service, and a system's markets. Shapes checked against
//! api.ardent-insight.com on 16 Oct 2026. An unknown system or service is HTTP 404
//! `{"error": "Not Found", "message": "System not found"}`.

use crate::{encode_segment, ttl, Client, Fetched, Request, Result, Service};
use serde::{Deserialize, Serialize};

/// Ardent caps `maxDistance` at 500 ly.
pub const MAX_DISTANCE_LY: f64 = 500.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trade {
    /// Stations selling the commodity (where to buy).
    Exports,
    /// Stations buying the commodity (where to sell).
    Imports,
}

/// Filters shared by the nearby-commodity searches. `None` leaves Ardent's default.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct MarketFilter {
    /// Clamped to [`MAX_DISTANCE_LY`].
    pub max_distance: Option<f64>,
    pub max_days_ago: Option<u32>,
    /// 1 small, 2 medium, 3 large.
    pub min_landing_pad: Option<u8>,
    pub fleet_carriers: Option<bool>,
    pub min_volume: Option<u64>,
}

impl MarketFilter {
    fn apply(&self, mut r: Request) -> Request {
        if let Some(d) = self.max_distance {
            r = r.query("maxDistance", d.clamp(0.0, MAX_DISTANCE_LY));
        }
        if let Some(d) = self.max_days_ago {
            r = r.query("maxDaysAgo", d);
        }
        if let Some(p) = self.min_landing_pad {
            r = r.query("minLandingPadSize", p);
        }
        if let Some(f) = self.fleet_carriers {
            r = r.query("fleetCarriers", f);
        }
        if let Some(v) = self.min_volume {
            r = r.query("minVolume", v);
        }
        r
    }
}

/// Station services Ardent's `/nearest/{service}` knows (all checked 16 Oct 2026).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StationService {
    InterstellarFactors,
    MaterialTrader,
    TechnologyBroker,
    BlackMarket,
    UniversalCartographics,
    Refuel,
    Repair,
    Shipyard,
    Outfitting,
    SearchAndRescue,
}

impl StationService {
    pub fn slug(self) -> &'static str {
        match self {
            StationService::InterstellarFactors => "interstellar-factors",
            StationService::MaterialTrader => "material-trader",
            StationService::TechnologyBroker => "technology-broker",
            StationService::BlackMarket => "black-market",
            StationService::UniversalCartographics => "universal-cartographics",
            StationService::Refuel => "refuel",
            StationService::Repair => "repair",
            StationService::Shipyard => "shipyard",
            StationService::Outfitting => "outfitting",
            StationService::SearchAndRescue => "search-and-rescue",
        }
    }
}

/// One commodity at one market.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Listing {
    pub commodity_name: String,
    pub market_id: i64,
    pub station_name: String,
    #[serde(default)]
    pub station_type: Option<String>,
    #[serde(default)]
    pub distance_to_arrival: Option<f64>,
    #[serde(default)]
    pub max_landing_pad_size: Option<u8>,
    pub system_address: i64,
    pub system_name: String,
    pub system_x: f64,
    pub system_y: f64,
    pub system_z: f64,
    pub buy_price: i64,
    pub sell_price: i64,
    pub stock: i64,
    pub demand: i64,
    #[serde(default)]
    pub mean_price: Option<i64>,
    pub updated_at: String,
    /// Light years from the searched system (nearby searches only).
    #[serde(default)]
    pub distance: Option<f64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NearestStation {
    pub market_id: i64,
    pub station_name: String,
    #[serde(default)]
    pub station_type: Option<String>,
    #[serde(default)]
    pub distance_to_arrival: Option<f64>,
    #[serde(default)]
    pub max_landing_pad_size: Option<u8>,
    pub system_address: i64,
    pub system_name: String,
    pub system_x: f64,
    pub system_y: f64,
    pub system_z: f64,
    pub distance: f64,
    #[serde(default)]
    pub updated_at: Option<String>,
}

fn system_path(system: &str) -> String {
    format!("/v2/system/name/{}", encode_segment(system))
}

impl Client {
    /// Stations near `system` that export (sell) or import (buy) `commodity`, nearest first.
    /// `commodity` is Ardent's id, e.g. "gold".
    pub fn ardent_nearby(
        &self,
        system: &str,
        commodity: &str,
        trade: Trade,
        filter: &MarketFilter,
    ) -> Result<Fetched<Vec<Listing>>> {
        let side = match trade {
            Trade::Exports => "exports",
            Trade::Imports => "imports",
        };
        let path = format!("{}/commodity/name/{}/nearby/{side}", system_path(system), encode_segment(commodity));
        self.fetch(Service::Ardent, &filter.apply(Request::get(path)), Some(ttl::MARKET))
    }

    /// Nearest stations to `system` with `service`, optionally with at least a pad size.
    pub fn ardent_nearest(
        &self,
        system: &str,
        service: StationService,
        min_landing_pad: Option<u8>,
    ) -> Result<Fetched<Vec<NearestStation>>> {
        let mut req = Request::get(format!("{}/nearest/{}", system_path(system), service.slug()));
        if let Some(p) = min_landing_pad {
            req = req.query("minLandingPadSize", p);
        }
        self.fetch(Service::Ardent, &req, Some(ttl::SEARCH))
    }

    /// Every commodity at every market in `system`.
    pub fn ardent_markets(&self, system: &str) -> Result<Fetched<Vec<Listing>>> {
        self.fetch(Service::Ardent, &Request::get(format!("{}/commodities", system_path(system))), Some(ttl::MARKET))
    }
}
