-- 2D mesh simulation
-- too demanding for real time, would likely never work

SAMPLE_RATE = 44100
BUFFER_LENGTH = 176400
SIZE_X = 10
SIZE_Y = 10
DISPERSION = 0.05
LOSS = 0.98
HALT = true
SINGLE_STEP = true

function love.load()
	love.window.setMode(640,480,{vsync=false})
	soundboard = {}
	local i = 0
	while i < SIZE_X*SIZE_Y do
		table.insert(soundboard, {z = 0, v = math.random(0,0)})
		i = i + 1
	end
	audio_buffer = love.sound.newSoundData(BUFFER_LENGTH, SAMPLE_RATE, 16, 1)
	cur_sample = 0
	
	string_data = love.sound.newSoundData("string.wav")
end

function love.update(dt)
	-- Do dispersion stuff
	if not HALT then
		local x, y = 0, 0
		for i,v in ipairs(soundboard) do
			if x+1 < SIZE_X then
				local next_point = x+2+(y*SIZE_X)
				local energy = (v.z-soundboard[next_point].z)*DISPERSION
				soundboard[next_point].v = soundboard[next_point].v + energy
				v.v = v.v - energy
			end
			if y+1 < SIZE_Y then
				local next_point = x+1+((1+y)*SIZE_X)
				local energy = (v.z-soundboard[next_point].z)*DISPERSION
				soundboard[next_point].v = soundboard[next_point].v + energy
				v.v = v.v - energy
			end
			
			if y == 0 or y+1 == SIZE_Y then
				v.z = 0
			elseif x == 0 or x+1 == SIZE_X then
				v.z = 0
			end

			x = x + 1
			if x >= SIZE_X then
				x = 0
				y = y + 1
			end
		end
		
		lost_energy = 0
		-- apply force
		for i,v in ipairs(soundboard) do
			lost_energy = lost_energy + ((v.z + v.v)-((v.z + v.v)*LOSS))
			v.z = (v.z + v.v)*LOSS
		end
		
		audio_buffer:setSample(cur_sample, lost_energy)
		
		soundboard[SIZE_X/2+((SIZE_Y/2)*SIZE_X)].z = string_data:getSample(cur_sample)
		cur_sample = cur_sample + 1
		
		if SINGLE_STEP then
			HALT = true
		end
	end
end

function love.draw()
	local x, y = 0, 0
	for i,v in ipairs(soundboard) do
		if v.z > 0 then
			love.graphics.setColor(v.z*255, 0, 0)
		elseif v.z < 0 then
			love.graphics.setColor(0, 0, v.z+1*255)
		else
			love.graphics.setColor(0,0,0)
		end
		love.graphics.points(x*2, y*2, x*2+1, y*2, x*2, y*2+1, x*2+1, y*2+1)
		x = x + 1
		if x >= SIZE_X then
			x = 0
			y = y + 1
		end
	end
end

function love.keypressed(key)
	if key == "r" then
		love.load()
	elseif key == "s" then
		if SINGLE_STEP then
			SINGLE_STEP = false
			HALT = false
		else
			SINGLE_STEP = true
			HALT = true
		end
	elseif key == "space" and SINGLE_STEP then
		HALT = false
	elseif key == "p" then
		love.audio.play(love.audio.newSource(audio_buffer))
	end
end

function round(num, numDecimalPlaces)
  return tonumber(string.format("%." .. (numDecimalPlaces or 0) .. "f", num))
end
