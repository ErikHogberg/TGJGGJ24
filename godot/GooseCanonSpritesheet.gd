extends Sprite2D
var cannonAim;
var hasDuckBeenLaunched;

# Called when the node enters the scene tree for the first time.
func _ready():
	cannonAim = 0;
	frame = 24;
	hasDuckBeenLaunched = false;
	pass # Replace with function body.


# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	print(cannonAim);
	if Input.is_action_just_pressed("AimUp")&& cannonAim < 4:
		_AimUp();

	if Input.is_action_just_pressed("AimDown")&& cannonAim >= 0: 
		_AimDown();
	
	if Input.is_action_pressed("jump")&& hasDuckBeenLaunched == false:
		_AnimChargeCannon();
	
	if Input.is_action_just_released("jump")&& hasDuckBeenLaunched == false:
		hasDuckBeenLaunched = true;
		$CannonAnimations.stop();
		_AnimShootCannon();
	pass


func _AimUp():
	cannonAim += 1;
	if cannonAim >= 3:
		cannonAim = 3;
	
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
		0: 
			frame = 24;
		1:
			frame = 16;
		2:
			frame = 8;
		3:
			frame = 0;
	pass

func _AnimChargeCannon():
	match cannonAim:
		1:
			$CannonAnimations.play("Charge3")
		2:
			$CannonAnimations.play("Charge2")
		3:
			$CannonAnimations.play("Charge1")
		4:
			$CannonAnimations.play("Charge0")
	pass


func _AnimShootCannon():
	match cannonAim:
		1:
			$CannonAnimations.play("CannonShoot3")
		2:
			$CannonAnimations.play("CannonShoot2")
		3:
			$CannonAnimations.play("CannonShoot1")
		4:
			$CannonAnimations.play("CannonShoot0")
	pass
