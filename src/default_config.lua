local posix = require "posix"
local unistd = require "posix.unistd"

local conf = {}

function conf.prompt()
        return unistd.getcwd() .. ": "
end

return conf
