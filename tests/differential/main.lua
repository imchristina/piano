-- Settings
SAMPLE_RATE = 48000
BUFFER_SIZE = 48000
SIN_TEST = false
RAND_TEST = false
NO_STRIKE = true
MANUAL_CONTROL = false
BLAST_PROCESSING = false

-- Constants
DELAY_LINE_SIZE = 250 -- number of points in string, turns out that it also acts as the length or tension of the string, as the sound gets higher frequency as this number gets lower
TRAVEL_SPEED = 2000
STRING_TENSION = 0.005
DISPERSION_COEFFICIENT = 0.3 -- energy transfered into surrounding points
TERMINATION_POINTS = 3
TERMINATION_FORCE = 1/TERMINATION_POINTS

function love.load()
	string = {}
	signal_graph = {}
	local i = 0
	while i < DELAY_LINE_SIZE do
		local value = 0
		if SIN_TEST then
			value = math.sin(i/10)*10
		elseif RAND_TEST then
			value = math.random(-3, 3)
		elseif NO_STRIKE then	
		else -- hammer model goes here, for now just a square wave sort of thing
			if i > 50 and i < 75 then
				value = math.random(9,10)
			else
				value = 0
			end
		end
		table.insert(string, {y = 0, v = value}) -- v = velocity
		table.insert(signal_graph, 0)
		i = i + 1
	end

	--audio_streaming = love.audio.newQueueableSource(SAMPLE_RATE, 16, 1)
	audio_buffer = love.sound.newSoundData(BUFFER_SIZE, SAMPLE_RATE, 16, 1)
	cur_sample = 0
end

function update_string(string, dt)
	local i = 1
	while i < DELAY_LINE_SIZE do -- disperse energy right
		string[i+1].y = string[i+1].y+(string[i].y*DISPERSION_COEFFICIENT) -- apply velocity from current point onto next point
		string[i].y = string[i].y-(string[i].y*DISPERSION_COEFFICIENT) -- remove applied velocity from point
		i = i + 1
	end
	local i = DELAY_LINE_SIZE
	while i > 1 do -- disperse energy left
		string[i-1].y = string[i-1].y+string[i].y*DISPERSION_COEFFICIENT
		string[i].y = string[i].y-string[i].y*DISPERSION_COEFFICIENT
		i = i - 1
	end

	for i,point in ipairs(string) do
		point.y = point.y + point.v -- apply velocity onto the string as movement
		point.v = point.v+(-point.y*STRING_TENSION) --set velocity from current position and tension of string
	end

	local i = 1
	while i <= TERMINATION_POINTS do --rigid terminations left
		string[(DELAY_LINE_SIZE+i-i*2)+1].y = string[(DELAY_LINE_SIZE+i-i*2)+1].y*(TERMINATION_FORCE*i)
		string[i].y = string[i].y*(TERMINATION_FORCE*i)
		i = i + 1
	end
	string[1].y = 0 -- make certain that root termination point is absolute unit 
	string[DELAY_LINE_SIZE].y = 0
	string[1].v = 0
	string[DELAY_LINE_SIZE].v = 0

	if cur_sample < BUFFER_SIZE then
		local sample = string[10].y/20
		if sample > 1 or sample < -1 then
			print("audio clipped")
		end
		audio_buffer:setSample(cur_sample, sample)
		cur_sample = cur_sample + 1
	end
	return string
end

function love.update(dt)
	if not MANUAL_CONTROL and BLAST_PROCESSING then
		local i = 0
		while i < TRAVEL_SPEED do
			string = update_string(string, dt)
			i = i + 1
		end
	elseif not MANUAL_CONTROL then
		string = update_string(string, dt)
	end

	if cur_sample == BUFFER_SIZE then
		love.audio.play(love.audio.newSource(audio_buffer))
		cur_sample = cur_sample + 1
	elseif cur_sample < BUFFER_SIZE then
		table.insert(signal_graph, string[10].y)
		table.remove(signal_graph, 1)
	end

	if love.keyboard.isDown("h") then
		local i = 10
		while i > 0 do
			string[i+30].y = -25
			i = i - 1
		end
	end
end

function love.draw()
	local i = 1
	while i <= DELAY_LINE_SIZE and i <= love.graphics.getWidth() do
		love.graphics.points(50+i, string[i].y+150)
		love.graphics.points(50+i, string[i].v+250)
		love.graphics.points(50+i, signal_graph[i]+350)
		love.graphics.points(50+(i), (audio_buffer:getSample((BUFFER_SIZE/DELAY_LINE_SIZE-1)*i))*50+450)
		i = i + 1
	end
end

function love.keypressed(key)
	if key == "r" then
		love.load()
	elseif key == "p" then
		cur_sample = cur_sample - 1
	elseif key == "=" or key == "+" then -- increase/decrease string tension
		STRING_TENSION = STRING_TENSION + 0.001
		print(STRING_TENSION)
		love.load()
	elseif key == "-" then
		STRING_TENSION = STRING_TENSION - 0.001
		print(STRING_TENSION)
		love.load()
	elseif key == "s" then -- slowmo
		TRAVEL_SPEED = 60
		love.load()
	elseif key == "f" then
		TRAVEL_SPEED = 200000
		love.load()
	elseif key == "space" then
		string = update_string(string, dt)
	end
end
