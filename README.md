## Scenario 1: Determining the Central Station
Consider an undirected graph with weights representing subway points and connections.
We need to define the central station, or the central vertex of the graph.
The central vertex is the one that can reach any of the other vertices with the lowest cost.
This takes into account both the sum of the distances from the vertex in question to each of the other vertices.\
### Expected output:
- The node representing the chosen central station;\
- A vector with the distances from the central station to the other vertices;\
- The vertex furthest from the central station, along with the distance value;\
- A matrix in which each row represents a candidate vertex for the central station and each column is the minimum distance between the candidate vertex and the vertex representing the column.\
## Scenario 2: Optimizing the Path with Regeneration
Consider an electric car with efficient battery regeneration via engine braking. The car
must travel from origin to destination while minimizing the net battery energy (Wh). Uphill/starting sections consume energy (positive weight). Downhill/regenerative braking sections return energy to the battery (negative weight).
We model the road network as a directed graph (energy is not symmetrical in either direction), and
the shape of the input graph is similar to that of the previous scenario.\
### Expected output:
- The shortest path, always starting from vertex 0 to vertex 6;\
- Sum of the path cost.\
## Scenario 3: Warehouse Robot with Obstacles
An inventory robot needs to travel from the reloading point (S) to the picking docking station (G)
inside a warehouse. There are shelves (obstacles), free aisles, and difficult floor areas
(e.g., areas with low pallets) with higher costs. The idea is to propose a graph search mechanism
that allows finding the lowest-cost path from S to G, avoiding
obstacles and preferring "cheap" aisles.\
### The map is defined as a grid, following the following rules:
- '=' free cell (cost 1);\
- "#" obstacle (impassable);\
- '~' difficult floor (cost 3);\
- 'S' = start;\
- 'G' = goal;\
- Movement in 4 directions (N, S, E, W).
