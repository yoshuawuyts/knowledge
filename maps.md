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
  - lan len -> place
  - place -> lat len
- route planner (driving directions)

### tile rendering
- take a map (roads, rivers and stuff)
- chop it into a grid at some resolution
- store each grid inside a table, and make it queryable
- each tile could be stored in a
  [quadtree](https://en.wikipedia.org/wiki/Quadtree)

## See Also
- [node-googlemaps](https://github.com/moshen/node-googlemaps)
- [developer.google.com/directions](https://developers.google.com/maps/documentation/directions/intro)
