//! Listener tests: replay fixture end to end, and axis annotations from a real preset's shape.
use astromech_input::listen::{binds_index, model, replay, summary, tracker};
use astromech_input::ELITE_TITLE;

#[test]
fn replay_fixture_end_to_end() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/replay-vkb.json");
    let mut p = tracker::Processor::new(ELITE_TITLE, None, true);
    let cap = replay::replay(&path, &mut p).unwrap();
    p.tick(1e12);
    let (focus, devs, over, anomalies, v) = summary::build(&p, &cap.timeline, cap.end_ms, Ok(()), &mut vec![]);
    assert_eq!(v[0].status, "WORKS");
    assert_eq!(focus.elite_focused_ms, 9300.0);
    let l = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D3201")).unwrap();
    assert_eq!(
        l.buttons.iter().map(|b| b.elite_name.clone().unwrap()).collect::<Vec<_>>(),
        vec!["Joy_4", "Joy_7", "Joy_12"]
    );
    assert_eq!(l.hats.len(), 2);
    assert_eq!(l.axes[0].movements, 2);
    let r = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D0200")).unwrap();
    assert_eq!(r.above_32, vec![33]);
    assert_eq!((r.events_elite_focused, r.events_not_focused), (5, 2));
    assert_eq!(over.len(), 1);
    assert_eq!(anomalies.len(), 1);
    assert!(anomalies[0].contains("Joy_7"));
}
/// Shape of Sumeet's DualVKB.4.2.binds: axes live under axis elements with a <Binding> plus
/// Inverted/Deadzone siblings; buttons use Primary/Secondary.
#[test]
fn axis_actions_from_his_preset_shape() {
    let d = tempfile::tempdir().unwrap();
    let x = r#"<?xml version="1.0" encoding="UTF-8" ?>
<Root PresetName="DualVKB" MajorVersion="4" MinorVersion="2">
	<YawAxisRaw>
		<Binding Device="231D0200" Key="Joy_RZAxis" />
		<Inverted Value="0" />
		<Deadzone Value="0.00000000" />
	</YawAxisRaw>
	<PitchAxisRaw>
		<Binding Device="231D0200" Key="Joy_YAxis" />
		<Inverted Value="1" />
		<Deadzone Value="0.00000000" />
	</PitchAxisRaw>
	<ThrottleAxis>
		<Binding Device="231D3201" Key="Joy_YAxis" />
		<Inverted Value="0" />
		<Deadzone Value="0.00000000" />
	</ThrottleAxis>
	<ForwardKey>
		<Primary Device="{NoDevice}" Key="" />
		<Secondary Device="231D3201" Key="Pos_Joy_RZAxis" />
	</ForwardKey>
</Root>"#;
    std::fs::write(d.path().join("DualVKB.4.2.binds"), x).unwrap();
    std::fs::write(d.path().join("StartPreset.4.start"), "DualVKB\nDualVKB\nDualVKB\nDualVKB\n").unwrap();
    let ix = binds_index::load(d.path(), Some(&d.path().join("nogame"))).unwrap();
    let mut p = tracker::Processor::new(ELITE_TITLE, Some(ix), true);
    for (k, pid) in [("l", 0x3201u16), ("r", 0x0200)] {
        p.add_device(model::DeviceMeta {
            key: k.into(),
            kind: model::DevKind::Hid,
            vid: 0x231D,
            pid,
            product: k.into(),
            usage: 4,
            buttons_declared: 128,
            axes_declared: vec!["Y".into(), "RZ".into()],
            hats_declared: 1,
        });
    }
    let s = |t: f64, dev: &str, y: f64, rz: f64| model::Sample {
        t_ms: t,
        dev: dev.into(),
        fg: ELITE_TITLE.into(),
        injected: false,
        data: model::SampleData::Hid {
            buttons: vec![],
            hats: vec![None],
            axes: vec![("Y".into(), y), ("RZ".into(), rz)],
        },
    };
    p.process(&s(0.0, "l", 0.5, 0.5));
    p.process(&s(0.0, "r", 0.5, 0.5));
    p.process(&s(100.0, "l", 0.9, 0.5));
    p.process(&s(100.0, "r", 0.5, 0.1));
    p.process(&s(200.0, "l", 0.5, 0.8));
    p.tick(5000.0);
    let (_, devs, _, _, _) = summary::build(&p, &[(0.0, ELITE_TITLE.into())], 5000.0, Ok(()), &mut vec![]);
    let l = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D3201")).unwrap();
    let y = l.axes.iter().find(|a| a.axis == "Y axis").unwrap();
    assert_eq!(y.actions, vec!["ThrottleAxis (Binding)"]);
    let rz = l.axes.iter().find(|a| a.axis == "RZ axis").unwrap();
    assert_eq!(rz.actions, vec!["ForwardKey (Secondary)"], "Pos_/Neg_ axis keys");
    let r = devs.iter().find(|d| d.elite_id.as_deref() == Some("231D0200")).unwrap();
    assert_eq!(r.axes.iter().find(|a| a.axis == "RZ axis").unwrap().actions, vec!["YawAxisRaw (Binding)"]);
}
