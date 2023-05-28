# Give Minecraft time to render everything the first time, about 10 ticks per iteration
execute if score Global ready matches 1 unless score Global paused matches 1 run function mapmaker:loop
execute unless score Global ready matches 1 unless score Global paused matches 1 run scoreboard players add Global ticker 1
execute unless score Global ready matches 1 unless score Global paused matches 1 if score Global ticker matches 10 run function mapmaker:render