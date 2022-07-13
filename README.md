# smatter

This was originally a project to create a 2d ship game based around giving orders to turrets on a ship to overcome a variety of threats. This project has been abandoned due to me losing interest/faith in the concept, and feeling that things could be structured much better for a future game based on what I learned from working on this. The current state of the project is a basic test case that spawns a ship with child turrets that automatically shoot at dummy targets, all with very placeholder assets.

This project is based on the Bevy engine.

Original readme follows:

# Gameplay Concept
Player ship automatically flies towards a stronghold, fighting off attacks the whole time. Player prepares for the fight by equipping weapons and upgrades to the ship before battle. During the battle the player gives orders to the ship, such as how fast to go (as waves are endless going slower will result in more enemies fought off total, and possibly a more well defended stronghold, but the ship doesn't have to fight off enemies as quickly. Some obstacles may be better tacked with speed, while others may be safer if done slowly.) The player can also give turrets general targeting orders, such as changing targeting preference weights between target distance, how far off the turret's current point of aim the target is, and target size. The player may also give targeting orders per enemy type, such as which sizes of turrets should target or ignore it. After each stronghold is defeated the player may choose a "Field Modification", giving the ship some bonus for the remainder of the run, and will also receive some currency, and/or some equipment (new guns, upgrades, turrets?) which will persist between battles and can be used to permanently upgrade the player's ship.

# The Fun
The enjoyment of the game comes from equipping and upgrading an increasingly powerful main ship, possibly with escorts, watching the ship fight through increasingly powerful waves of enemies using player given orders, and trying to get the best possible run using field upgrades to overcome increasingly unfair odds.

# Standards

## Z-Layers
- The ship owns layers 1-10, 5 nominal.
- Projectiles owns layers 11-20, 15 nominal.
- Turrets owns layers 21-30, 25 nominal.