--THINGS MISSING
--Loss in string (scattering junction)
--A crude hammer model
--Accurate frequency control
--Frequency accuracy at different sample rates

-- Settings
SAMPLE_RATE = 48000
BUFFER_SIZE = 48000
SIN_TEST = false
RAND_TEST = false

-- Constants
DELAY_LINE_SIZE = 500 -- number of points in string, turns out that it also acts as the length or tension of the string, as the sound gets higher frequency as this number gets lower
TRAVEL_SPEED = 200000 -- number of points shifted per second, higher numbers increase CPU usage but make sampling complete faster
REFLECTION_COEFFICIENT = 0.93

function love.load()
	dwgs = {l = {}, r = {}}
	signal_graph = {}
	string_sum = {}
	local i = 0
	while i < DELAY_LINE_SIZE do
		local l_value, r_value = 0
		if SIN_TEST then
			l_value = math.sin(i/10)*10
			r_value = math.sin(i/10)*10
		elseif RAND_TEST then
			l_value = math.random(-10, 10)
			r_value = math.random(-10, 10)
		else -- hammer model goes here, for now just a square wave sort of thing
			if i > 50 and i < 75 then
				l_value = 10
				r_value = 10
			else
				l_value = 0
				r_value = 0
			end
		end
		table.insert(dwgs.l, l_value)
		table.insert(dwgs.r, r_value)
		table.insert(string_sum, 0)
		table.insert(signal_graph, 0)
		i = i + 1
	end

	--audio_streaming = love.audio.newQueueableSource(SAMPLE_RATE, 16, 1)
	audio_buffer = love.sound.newSoundData(BUFFER_SIZE, SAMPLE_RATE, 16, 1)
	cur_sample = 0
end

function update_string(l, r, dt)
	local points_to_move = TRAVEL_SPEED*dt
	while points_to_move > 0 do
		table.insert(r, 1, -(l[1]*REFLECTION_COEFFICIENT))
		table.insert(l, -(r[DELAY_LINE_SIZE]*REFLECTION_COEFFICIENT))
		table.remove(r, table.maxn(r))
		table.remove(l, 1);
		points_to_move = points_to_move - 1

		if cur_sample < BUFFER_SIZE then
			local sample = (dwgs.r[1]+dwgs.l[DELAY_LINE_SIZE])/20
			if sample > 1 or sample < -1 then
				print("audio clipped")
			end
			audio_buffer:setSample(cur_sample, sample)
			cur_sample = cur_sample + 1
		end
	end
	return l, r
end

function love.update(dt)
	dwgs.l, dwgs.r = update_string(dwgs.l, dwgs.r, dt)

	string_sum = {}
	local i = 1
	while i <= DELAY_LINE_SIZE do
		string_sum[i] = dwgs.r[i] + dwgs.l[i]
		i = i + 1
	end

	if cur_sample == BUFFER_SIZE then
		love.audio.play(love.audio.newSource(audio_buffer))
		cur_sample = cur_sample + 1
	elseif cur_sample < BUFFER_SIZE then
		table.insert(signal_graph, dwgs.r[1]+dwgs.l[DELAY_LINE_SIZE])
		table.remove(signal_graph, 1)
	end
end

function love.draw()
	local i = 1
	while i <= DELAY_LINE_SIZE and i <= love.graphics.getWidth() do
		love.graphics.points(50+i, dwgs.r[i]+50)
		love.graphics.points(50+i, dwgs.l[i]+150)
		love.graphics.points(50+i, string_sum[i]+250)
		if cur_sample < BUFFER_SIZE then
			love.graphics.points(50+i, signal_graph[i]+350)
		else
			love.graphics.points(50+(i), (audio_buffer:getSample((BUFFER_SIZE/DELAY_LINE_SIZE-1)*i))*50+350)
		end
		i = i + 1
	end
end

function love.keypressed(key)
	if key == "r" then
		love.load()
	elseif key == "p" then
		cur_sample = cur_sample - 1
	elseif key == "=" or key == "+" then -- increase/decrease string length/frequency
		DELAY_LINE_SIZE = DELAY_LINE_SIZE + 10
		love.load()
	elseif key == "-" then
		DELAY_LINE_SIZE = DELAY_LINE_SIZE - 10
		love.load()
	elseif key == "s" then -- slowmo
		TRAVEL_SPEED = 60
		love.load()
	elseif key == "f" then
		TRAVEL_SPEED = 200000
		love.load()
	end
end
