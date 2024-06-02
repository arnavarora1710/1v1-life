# 1v1-life
A two player version of Conway's Game of Life: given two starting states and a set of cellular automata rules, find the winner as the state which ends the last. 

A typical setup of the Game of Life has two sets of rules: Birth and Survival, i.e., a cell gets populated if the Birth rules are obeyed for that cell, a cell dies if the Survival rules are not obeyed for that cell. 

I will be following some of the rules defined in [P2Life](https://www.dcs.bbk.ac.uk/~gr/pdf/p2life.pdf) as they work nicely with game-theoretic and field theory domains and will also try adding some fun user-defined customizations. Planning to implement and deploy this fully in Rust with [Macroquad](https://macroquad.rs/) and [Shuttle](https://www.shuttle.rs/)!
