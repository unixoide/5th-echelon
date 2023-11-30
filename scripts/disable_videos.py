import pathlib

DIR = pathlib.Path(r"C:\Program Files (x86)\Steam\steamapps\common\Tom Clancy's Splinter Cell Blacklist\data\videos")

VIDEOS = [
    "Copyright.bik",
    "Disclaimer.bik",
    "Logo_MiddleWare_01.bik",
    "Logo_Ubisoft_Clancy.bik",
]

for root in [DIR] + [p for p in DIR.iterdir() if p.is_dir()]:
    for vid_name in VIDEOS:
        vid = root / vid_name
        if vid.exists():
            target = vid.with_suffix(vid.suffix + ".disabled")
            vid.replace(target)
            print("Renamed", vid, "to", target)