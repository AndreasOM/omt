local lastStepDistance = 0

function nextStep()
	lastStepDistance = getDistanceInMeters()
	print("nextStep @", lastStepDistance)
end

function initialize()
	print("initialize")
--	doNotCallMe()
	nextStep()
end
