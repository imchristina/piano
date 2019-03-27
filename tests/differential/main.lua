-- Put on protective gear before looking at this garbage

-- Settings
SAMPLE_RATE = 96000
BUFFER_SIZE = 441000
SIN_TEST = false
RAND_TEST = true
NO_STRIKE = false
MANUAL_CONTROL = false
BLAST_PROCESSING = true
SAMPLE_POINT = true -- sample string position if false, velocity if true

-- Constants
DELAY_LINE_SIZE = 500 -- number of points in string, turns out that it also acts as the length or tension of the string, as the sound gets higher frequency as this number gets lower
TRAVEL_SPEED = 2000
DISPERSION_COEFFICIENT = 1 -- energy transfered into surrounding points
TERMINATION_POINTS = 3
TERMINATION_FORCE = 1/TERMINATION_POINTS

function love.load()
	string = {}
	signal_graph = {}
	local i = 0
	while i < DELAY_LINE_SIZE do
		local value = 0
		if SIN_TEST then
			value = math.sin(i/10)
		elseif RAND_TEST then
			value = math.random(-1, 1)
		elseif NO_STRIKE then	
		else -- hammer model goes here, for now just a square wave sort of thing
			if i > 10 and i < 15 then
				value = -1--math.random(-10, -10)
			else
				value = 0
			end
		end
		table.insert(string, {y = 0, v = value}) -- v = velocity
		table.insert(signal_graph, 0)
		i = i + 1
	end

	audio_buffer = love.sound.newSoundData(BUFFER_SIZE, SAMPLE_RATE, 16, 1)
	cur_sample = 0
end

function update_string(string, dt)
	local i = 1
	while i < DELAY_LINE_SIZE do -- disperse energy right
		local energy = (string[i].y-string[i+1].y)*DISPERSION_COEFFICIENT
		string[i+1].v = string[i+1].v + energy
		string[i].v = string[i].v - energy
		i = i + 1
	end

	for i,point in ipairs(string) do
		point.y = point.y + point.v -- apply velocity onto the string as movement
	end

	local i = 1
	while i <= TERMINATION_POINTS do --rigid terminations
		string[i].y = string[i].y*(TERMINATION_FORCE*i)
		i = i + 1
	end
	local sample = string[1].y
	if SAMPLE_POINT then
		sample = string[1].v
	end
	string[1].y = 0 -- make certain that root termination point is absolute unit 
	string[DELAY_LINE_SIZE].y = 0
	string[1].v = 0
	string[DELAY_LINE_SIZE].v = 0

	if cur_sample < BUFFER_SIZE then
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
	elseif cur_sample < BUFFER_SIZE and cur_sample > 0 then
		table.insert(signal_graph, audio_buffer:getSample(cur_sample-1)*25)
		table.remove(signal_graph, 1)
	end

	if love.keyboard.isDown("h") and NO_STRIKE then
		local i = 10
		while i > 0 do
			string[i+30].v = string[i+30].v - 1
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
		DISPERSION_COEFFICIENT = DISPERSION_COEFFICIENT + 0.1
		print(DISPERSION_COEFFICIENT)
		love.load()
	elseif key == "-" then
		DISPERSION_COEFFICIENT = DISPERSION_COEFFICIENT - 0.1
		print(DISPERSION_COEFFICIENT)
		love.load()
	elseif key == "right" then
		DELAY_LINE_SIZE = DELAY_LINE_SIZE + 50
		love.load()
	elseif key == "left" then
		DELAY_LINE_SIZE = DELAY_LINE_SIZE - 50
		love.load()
	elseif key == "s" then -- slowmo
		BLAST_PROCESSING = false
	elseif key == "f" then
		BLAST_PROCESSING = true
	elseif key == "space" then
		string = update_string(string, dt)
	end
end
