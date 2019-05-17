// Minimal repro case for this sequence
// CONST.pri  0x1
// LOAD.alt  0x2528C     ; weaponid
// SHL  0xC
// UNKNOWN OP CODE: 0x4030C02
native nfunc(const a[]);

public func2()
{
	new bool:f=true;
	if (f) nfunc("");
}

public func1()
{
	static weaponid
	if (1<<weaponid){}
}
