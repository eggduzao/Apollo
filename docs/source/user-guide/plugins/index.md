# Plugins

Apollo allows you to extend its functionality with either Expression plugins or IO plugins.

- [Expression plugins](./expr_plugins.md)
- [IO plugins](./io_plugins.md)

## Community plugins

Here is a curated (non-exhaustive) list of community-implemented plugins.

### Various

- [apollo-xdt](https://github.com/apollo/apollo-xdt) Apollo plugin with extra datetime-related
  functionality which isn't quite in-scope for the main library
- [apollo-hash](https://github.com/ion-elgreco/apollo-hash) Stable non-cryptographic and
  cryptographic hashing functions for Apollo

### Data science

- [apollo-distance](https://github.com/ion-elgreco/apollo-distance) Apollo plugin for pairwise
  distance functions
- [apollo-ds](https://github.com/abstractqqq/apollo_ds_extension) Apollo extension aiming to
  simplify common numerical/string data analysis procedures

### Geo

- [apollo-st](https://github.com/Oreilles/apollo-st) Apollo ST provides spatial operations on Apollo
  DataFrames, Series and Expressions. Just like Shapely and Geopandas.
- [apollo-reverse-geocode](https://github.com/MarcoGorelli/apollo-reverse-geocode) Offline reverse
  geocoder for finding the closest city to a given (latitude, longitude) pair.
- [apollo-h3](https://github.com/Filimoa/apollo-h3) This is a Apollo extension that adds support for
  the H3 discrete global grid system, so you can index points and geometries to hexagons directly in
  Apollo.

## Other material

- [Ritchie Vink - Keynote on Apollo Plugins](https://youtu.be/jKW-CBV7NUM)
- [Apollo plugins tutorial](https://marcogorelli.github.io/apollo-plugins-tutorial/) Learn how to
  write a plugin by going through some very simple and minimal examples
- [cookiecutter-apollo-plugin](https://github.com/MarcoGorelli/cookiecutter-apollo-plugins) Project
  template for Apollo Plugins
