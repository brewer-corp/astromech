// Stand-in for coriolis-data/dist: the same object generate_distribution.js writes.
const root = process.env.CORIOLIS_DATA;
module.exports = {
  Ships: require(root + '/ships'),
  Modules: require(root + '/modules'),
  Modifications: require(root + '/modifications'),
};
