extends Sprite2D


# Called when the node enters the scene tree for the first time.
func _ready():
	fill()

func decrease():
	frame+=1
	if frame> 12:
		frame = 12

func fill():
	frame = 0
