local _ident = 0
function ident()
	_ident = _ident + 1
end
function deent()
	_ident = _ident - 1
end
function bprint(smth)
	local buf = ""
	for i = 1, _ident do
		buf = buf .. ".      "
	end
	buf = buf .. smth
	print(buf)
end

function OnBeforeCompilation()
	local file = vmprotect.core():outputArchitecture()
	for i = 1, file:mapFunctions():count() do
		local fn = file:mapFunctions():item(i)
		if fn:name():find("^vmprotect_") ~= nil then
			bprint("Added " .. fn:name())
			ident()
			local added
			local addType
			if fn:name():find("_ultra_") ~= nil then
				bprint("As ultra")
				addType = CompilationType.Ultra
			elseif fn:name():find("_virtualize_") ~= nil then
				bprint("As virtualized")
				addType = CompilationType.Virtualized
			elseif fn:name():find("_mutate_") ~= nil then
				bprint("As mutated")
				addType = CompilationType.Mutate
			end
			added = file:functions():addByAddress(fn:address(), addType)
			if fn:name():find("_lock_") ~= nil then
				if addType == CompilationType.Mutate then
					error("Lock doesn't work at mutated code!")
				end
				bprint("And locked by key")
				added:setLockToKey(true)
			end
			deent()
		end
	end
end