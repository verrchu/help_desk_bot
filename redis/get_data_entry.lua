local key = ARGV[1]
local lang = ARGV[2]

local call_key = string.gsub('$key:data:$lang', '%$(%w+)', {key=key, lang=lang})
local data = redis.pcall('get', call_key)

local call_key = string.gsub('$key:created', '%$(%w+)', {key=key})
local created = redis.pcall('get', call_key)

local call_key = string.gsub('$key:views', '%$(%w+)', {key=key})
local views = redis.pcall('get', call_key)

local call_key = string.gsub('$key:likes', '%$(%w+)', {key=key})
local likes = redis.pcall('scard', call_key)

return {
    'text', string.gsub(data, '^%s*(.-)%s*$', '%1'),
    'created', created,
    'views', views,
    'likes', tostring(likes)
}
