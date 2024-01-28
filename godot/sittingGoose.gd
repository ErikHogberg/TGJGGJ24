extends Area2D

func show_goose():
	$SittingGoose.show()
	monitoring = true
	pass
func hide_goose():
	$SittingGoose.hide()
	monitoring = false
	pass
	
func hit(asdf):
	print("hit")
	%Player.pause()
	$GooseSoundHit.play()
