--[[

Hybrid String Simulation

Using a waveguide and differential-ish techniques
to simulate a string with okay efficiency and no accuracy whatsoever

]]

--Simulation constants
WAVEGUIDE_LENGTH = 500
DIFFERENTIAL_LENGTH = 10
SOFT_TERMINATION_POINTS = 3
SOFT_TERMINATION_FORCE = 0.33
LOSS_COEFFICIENT = 0.9999
INITIAL_DISPLACEMENT = "rand"
SIM_SPEED_FAST = 2000
SIM_SPEED = 1

-- Recording constants
RECORD_LENGTH = 960000
SAMPLE_RATE = 96000
RECORD_GAIN = 0.5

-- UI constants
LENGTH_CHANGE_SIZE = 50
MIN_LENGTH = 10
DISPLAY_POS = {x = 150, y = 100}
DISPLAY_OFFSET = 100
DISPLAY_SCALE = {x = 1, y = 10}

-- Debug mode
-- Prints output of update_string() using set parameters
-- In CSV format to be piped into file
DEBUG_ENABLE = false
DEBUG_SAMPLES = 1024

if DEBUG_ENABLE then
	WAVEGUIDE_LENGTH = 10
	DIFFERENTIAL_LENGTH = 10
	SOFT_TERMINATION_POINTS = 3
	SOFT_TERMINATION_FORCE = 0.33
	LOSS_COEFFICIENT = 1.0
	INITIAL_DISPLACEMENT = "point"
	SIM_SPEED_FAST = 2000
	SIM_SPEED = 1
end


function love.load()
	waveguide = {}
	local i = 0
	while i < WAVEGUIDE_LENGTH do
		table.insert(waveguide, 0)
		i = i + 1
	end
	
	differential = {}
	local i = 0
	while i < DIFFERENTIAL_LENGTH do
		table.insert(differential, {d = 0, v = 0})
		i = i + 1
	end
	
	-- Initial Displacement/Strike
	if INITIAL_DISPLACEMENT == "point" then
		waveguide[1] = 1
	elseif INITIAL_DISPLACEMENT == "rand" then
		for i,v in ipairs(waveguide) do
			waveguide[i] = math.random(-1, 1)
		end
	end
	
	if DEBUG_ENABLE then
		local i = 0
		while i < DEBUG_SAMPLES do
			io.write(tostring(update_string())..",")
			i = i + 1
		end
		os.exit()
	end
	
	audio_buffer = love.sound.newSoundData(RECORD_LENGTH, SAMPLE_RATE, 16, 1)
	sample_index = 1
end

function love.update(dt)
	local i = SIM_SPEED
	while i > 0 do
		local sample = update_string()
		if sample_index < RECORD_LENGTH then
			audio_buffer:setSample(sample_index, sample*RECORD_GAIN)
			sample_index = sample_index + 1
		end
		i = i - 1
	end
end

function update_string()
	-- Get inverted value from waveguide to simulate second inverting termination
	local waveguide_out = -waveguide[1]
	
	-- Inject value from waveguide near the end of differential
	differential[DIFFERENTIAL_LENGTH-1].d = differential[DIFFERENTIAL_LENGTH-1].d + waveguide_out
	differential[DIFFERENTIAL_LENGTH-1].v = differential[DIFFERENTIAL_LENGTH-1].v + waveguide_out
	differential[DIFFERENTIAL_LENGTH].v = differential[DIFFERENTIAL_LENGTH].v - waveguide_out
	
	-- Get difference, apply to velocity
	local i = 1
	while i < DIFFERENTIAL_LENGTH do
		local difference = differential[i].d-differential[i+1].d
		differential[i+1].v = differential[i+1].v + difference
		differential[i].v = differential[i].v - difference
		i = i + 1
	end
	
	-- Apply velocity as displacement
	for i,point in ipairs(differential) do
		point.d = point.d + point.v
	end
	
	-- Apply soft termination to end
	local i = 1
	while i <= SOFT_TERMINATION_POINTS do
		differential[i].d = differential[i].d*(SOFT_TERMINATION_FORCE*i)
		i = i + 1
	end
	
	-- Rigid termination
	differential[1].d = 0
	differential[1].v = 0
	
	-- Get sample, remove sampled value from differential
	local sample = differential[DIFFERENTIAL_LENGTH].d
	differential[DIFFERENTIAL_LENGTH].d = 0
	differential[DIFFERENTIAL_LENGTH].v = differential[DIFFERENTIAL_LENGTH].v - sample
	differential[DIFFERENTIAL_LENGTH-1].v = differential[DIFFERENTIAL_LENGTH-1].v + sample
	
	-- Small loss to help counteract DC offset issue
	-- There's probably a better way to fix this
	sample = sample * LOSS_COEFFICIENT
	
	-- Insert sample into waveguide
	table.remove(waveguide, 1)
	table.insert(waveguide, sample)
	
	return sample
end

function love.keypressed(key)
	if key == "r" then
		love.load()
	elseif key == "+" or key == "=" then
		WAVEGUIDE_LENGTH = WAVEGUIDE_LENGTH + LENGTH_CHANGE_SIZE
		local i = LENGTH_CHANGE_SIZE
		while i > 0 do
			table.insert(waveguide, 0)
			i = i - 1
		end
	elseif (key == "-" or key == "_") and WAVEGUIDE_LENGTH-LENGTH_CHANGE_SIZE > MIN_LENGTH then
		WAVEGUIDE_LENGTH = WAVEGUIDE_LENGTH - LENGTH_CHANGE_SIZE
		local i = LENGTH_CHANGE_SIZE
		while i > 0 do
			table.remove(waveguide, 1)
			i = i - 1
		end
	elseif key == "f" then
		SIM_SPEED = SIM_SPEED_FAST
	elseif key == "s" then
		SIM_SPEED = 1
	elseif key == "p" then
		love.audio.play(love.audio.newSource(audio_buffer))
	end
end

function love.draw()
	love.graphics.print("Hybrid String | R - Reload, +/- - Change Size, F/S - Sim Speed, P - Play Audio", 10, 10)
	love.graphics.print("Displacement", 10, DISPLAY_POS.y)
	love.graphics.print("Velocity", 10, DISPLAY_POS.y+DISPLAY_OFFSET)
	for i,v in ipairs(differential) do
		love.graphics.points(DISPLAY_POS.x+i*DISPLAY_SCALE.x, DISPLAY_POS.y+-v.d*DISPLAY_SCALE.y)
		love.graphics.points(DISPLAY_POS.x+i*DISPLAY_SCALE.x, DISPLAY_POS.y+-v.v*DISPLAY_SCALE.y+DISPLAY_OFFSET)
	end
	love.graphics.print("Waveguide", 10, DISPLAY_POS.y+DISPLAY_OFFSET*2)
	for i,v in ipairs(waveguide) do
		love.graphics.points(DISPLAY_POS.x+i*DISPLAY_SCALE.x, DISPLAY_POS.y+-v*DISPLAY_SCALE.y+DISPLAY_OFFSET*2)
	end
	love.graphics.print("Seconds Audio Buffered (Max "..tostring(RECORD_LENGTH/SAMPLE_RATE).."): "..tostring(sample_index/SAMPLE_RATE), 10, DISPLAY_POS.y+DISPLAY_OFFSET*3)
end
