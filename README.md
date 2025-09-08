## Scenario 1: Determining the Central Station
Consider an undirected graph with weights representing subway points and connections.
We need to define the central station, or the central vertex of the graph.
The central vertex is the one that can reach any of the other vertices with the lowest cost.
This takes into account both the sum of the distances from the vertex in question to each of the other vertices.\
Expected output:\
The node representing the chosen central station;\
A vector with the distances from the central station to the other vertices;\
The vertex furthest from the central station, along with the distance value;\
A matrix in which each row represents a candidate vertex for the central station and each;\
column is the minimum distance between the candidate vertex and the vertex representing the column.\
