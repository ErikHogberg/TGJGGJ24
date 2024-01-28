extends Sprite2D

@export var tiling: int = 1
@export var tiling_offset: int = 0

var reset_x;

func _ready():
	var width = get_rect().size.x * scale.x
	global_position.x = width * tiling_offset
	reset_x = position.x
	
func restart():
	position.x = reset_x

func _process(delta):
	var width = get_rect().size.x * scale.x
	var halfWidth = width * 0.5
	var x = global_position.x
	var left = x - halfWidth
	var right = x + halfWidth
	
	if right < -width*3.0 :
		global_position.x += width * tiling
		#print("moved")

