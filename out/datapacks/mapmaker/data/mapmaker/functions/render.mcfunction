scoreboard players set Global ticker 0
function mapmaker:loop
scoreboard players add Global rendering 1
execute if score Global rendering > Global frames run scoreboard players set Global ready 1