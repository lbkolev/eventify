--[[
Function calls in the Ethereum Virtual Machine are specified by the first four bytes of data sent with a transaction.
These 4-byte signatures are defined as the first four bytes of the Keccak hash (SHA3) of the canonical representation of the function signature.

Fetch all submitted/well-known Function Signatures from the public database https://4byte.directory & save them in a csv that'll be imported during the initial execution of the eventify binary (or db_init).
]]

function script_directory()
    local path = arg[0]
    local dir = path:match("(.*/)")

    if dir == nil then
        return "./"
    end

    return dir
end

local https = require("ssl.https")
local json = require("dkjson")
local argparse = require("argparse")

--- parse arguments
local parser = argparse("Fetch https://4byte.directory Function Signatures into a csv")
parser:option("-f --from-page", "Start from page", "1")
parser:option("-t --to-page", "End at page", "10000000")
parser:option("-o --output", "Output file",
    string.format("%s../migrations/data/function_signatures.csv", script_directory()))
local args = parser:parse()
print(string.format("Fetching pages %s - %s into file '%s'.", args.from_page, args.to_page, args.output))

--- setup the starting point
ENDPOINT = "http://www.4byte.directory:443/api/v1/signatures/?page=" .. args.from_page

--- cleanup the output file
io.open(args.output, "w"):close()

while true do
    print(string.format("Requesting %s", ENDPOINT))
    local response, status = https.request(ENDPOINT)

    if status ~= 200 then
        print("HTTPS request failed with status: " .. status)
        break
    end

    local json_data, _ = json.decode(response)
    local file = io.open(args.output, "a")
    ENDPOINT = json_data["next"]

    for _, v in pairs(json_data["results"]) do
        -- as we're using the csv to import into postgres, convertion from 0x to \x is required
        file:write(string.format("%s,\"%s\"\n", v["hex_signature"]:gsub("^0x", "\\x"), v["text_signature"]))
    end
    file:close()

    if json_data["next"] == nil or json_data["next"] == "http://www.4byte.directory:443/api/v1/signatures/?page=" .. args.to_page then
        print("reached last page")
        break
    end
end
