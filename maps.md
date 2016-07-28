# maps
Creating maps, routing and more.

## directions
### using google maps
1. create a google project key. Can be done on the maps developer home page.
2. pull in the `googlemaps` client from npm

## Implementing maps
Consists of 3 elements:
- tile renderer
- geo coder (like a table)
  - lan len -> place (k-d tree)
  - place -> lat len
- route planner (driving directions)

### tile rendering
- take a map (roads, rivers and stuff)
- chop it into a grid at some resolution
- store each grid inside a table, and make it queryable
- each tile could be stored in a
  [quadtree](https://en.wikipedia.org/wiki/Quadtree)

### geocoder
- use a [k-d tree](https://en.wikipedia.org/wiki/K-d_tree) for lat-len -> place
- use a [binary search tree](https://en.wikipedia.org/wiki/Binary_search_tree)
  for place -> lat-len

### route planner
- start off with an Open Street Map data set. The OSM data set has stuff
  embedded like speed limits and lengths and stuff. Nodes are rougly
  proportional to places. Enables calculating transit time

## See Also
- [node-googlemaps](https://github.com/moshen/node-googlemaps)
- [developer.google.com/directions](https://developers.google.com/maps/documentation/directions/intro)
