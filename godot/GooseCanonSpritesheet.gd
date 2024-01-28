extends Sprite2D
var cannonAim;

# Called when the node enters the scene tree for the first time.
func _ready():
	cannonAim = 0;
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if Input.is_action_just_pressed("AimUp")&& cannonAim < 5:
		_AimUp();

	if Input.is_action_just_pressed("AimDown")&& cannonAim >= 0: 
		_AimDown();
	pass


func _AimUp():
	cannonAim += 1;
	if cannonAim >= 5:
		cannonAim = 4;
	
	_AnimCannon();
	pass

func _AimDown():
	cannonAim -= 1;
	if cannonAim <= 0:
		cannonAim = 0;

	_AnimCannon();
	pass

func _AnimCannon():
	match cannonAim:
		1: 
			frame = 24;
		2:
			frame = 16;
		3:
			frame = 8;
		4:
			frame = 0;

	pass
