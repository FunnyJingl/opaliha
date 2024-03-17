from pydantic import BaseModel


class Material(BaseModel):
    name: str = "air"


class Glass(Material):
    def __init__(self, name: str):
        super().__init__(name=name)


class GlassCatalog:
    def __init__(self, name: str):
        self._name = name
        self._catalog: dict[str, Glass] = {}

    def from_agf(self, path: str):
        catalog = read_agf(path=path)
        for k, v in catalog.items():
            self._catalog[k] = Glass(name=k)


def read_agf(path: str) -> dict:
    with open(path, 'r', encoding='latin1') as f:
        glass_catalog = {}
        for nline, line in enumerate(f):
            if not line.strip():
                continue
            if line.startswith('CC '):
                continue
            if line.startswith('NM '):
                nm = line.split()
                glassname = nm[1]
                glass_catalog[glassname] = {}
                glass_catalog[glassname]['dispform'] = read_from_engineering_int(nm[2])
                glass_catalog[glassname]['nd'] = float(nm[4])
                glass_catalog[glassname]['vd'] = float(nm[5])
                glass_catalog[glassname]['exclude_sub'] = 0 if (len(nm) < 7) else read_from_engineering_int(nm[6])
                glass_catalog[glassname]['status'] = 0 if (len(nm) < 8) else read_from_engineering_int(nm[7])
                glass_catalog[glassname]['meltfreq'] = 0 if (len(nm) < 9) or (nm.count('-') > 0) else read_from_engineering_int(nm[8])
            elif line.startswith('ED '):
                ed = line.split()
                glass_catalog[glassname]['tce'] = float(ed[1])
                glass_catalog[glassname]['density'] = float(ed[3])
                glass_catalog[glassname]['dpgf'] = float(ed[4])
                glass_catalog[glassname]['ignore_thermal_exp'] = 0 if (len(ed) < 6) else read_from_engineering_int(ed[5])
            elif line.startswith('CD '):
                cd = line.split()[1:]
                glass_catalog[glassname]['cd'] = [float(a) for a in cd]
            elif line.startswith('TD '):
                td = line.split()[1:]
                if not td:
                    continue  # sometimes an empty line occurs for the "TD" label
                glass_catalog[glassname]['td'] = [float(a) for a in td]
            elif line.startswith('OD '):
                line = line.strip()
                line = line.replace('   ', ' - ')
                od = line.split()[1:]
                od = string_list_to_float_list(od)
                glass_catalog[glassname]['relcost'] = od[0]
                glass_catalog[glassname]['cr'] = od[1]
                glass_catalog[glassname]['fr'] = od[2]
                glass_catalog[glassname]['sr'] = od[3]
                glass_catalog[glassname]['ar'] = od[4]
                glass_catalog[glassname]['pr'] = od[5] if len(od) == 6 else -1.0
            elif line.startswith('LD '):
                ld = line.split()[1:]
                glass_catalog[glassname]['ld'] = [float(a) for a in ld]
            elif line.startswith('IT '):
                it = line.split()[1:]
                it_row = [float(a) for a in it]
                if 'it' not in glass_catalog[glassname]:
                    glass_catalog[glassname]['IT'] = {}
                glass_catalog[glassname]['IT']['wavelength'] = it_row[0]
                glass_catalog[glassname]['IT']['transmission'] = it_row[1]
                glass_catalog[glassname]['IT']['thickness'] = it_row[2] if len(it_row) > 2 else None
    return glass_catalog


def read_from_engineering_int(x: str) -> int:
    float_x = float(x)
    val = int(float_x)
    assert val == float_x
    return val


def string_list_to_float_list(x):
    npts = len(x)
    if (npts == 0) or ((npts == 1) and (x[0].strip() == '-')):
        return [-1.0] * 10

    res = []
    for a in x:
        if a.strip() == '-':
            res.append(-1.0)
        else:
            try:
                res.append(float(a))
            except ValueError:
                res.append(None)
    return res
