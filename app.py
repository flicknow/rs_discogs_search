import requests
import gzip
import xml.etree.ElementTree as ElementTree
from functools import partial
from pprint import pprint
import time

def skip_tag(ctx, tag):
    for event, elem in ctx:
        if elem.tag == tag and event == "end":
            return

def parse_node_type(ctx, tag, stop, parser):
    for event, elem in ctx:
        if event == "start" and elem.tag == tag:
            yield parser(ctx)
        elif event == "end" and elem.tag == stop:
            break
        else:
            raise Exception("unexpected event and element: (%s, %s)" % (event, elem.tag))

def parse_node_text(tag, ctx):
    for event, elem in ctx:
        if event == "end" and elem.tag == tag:
            return elem.text

def parse_artists_stream(ctx):
    return parse_node_type(ctx, "artist", "artists", parse_artist)

def parse_labels_stream(ctx):
    return parse_node_type(ctx, "label", "labels", parse_label)

def parse_masters_stream(ctx):
    return parse_node_type(ctx, "master", "masters", parse_master)

def parse_releases_stream(ctx):
    return parse_node_type(ctx, "release", "releases", parse_release)

def parse_artist(ctx):
    artist={}
    for event, elem in ctx:
        if event == "start":
            match elem.tag:
                case "id":
                    artist["id"] = elem.text
                case "name":
                    artist["name"] = elem.text
                case "realname":
                    artist["realname"] = elem.text
                case "namevariations":
                    artist["namevariations"] = [name for name in parse_node_type(ctx, "name", "namevariations", partial(parse_node_text, "name"))]
                case "aliases":
                    artist["aliases"] = [alias for alias in parse_node_type(ctx, "name", "aliases", parse_artist_alias)]
                case "profile":
                    artist["profile"] = elem.text
                case _:
                    skip_tag(ctx, elem.tag)
        elif event == "end" and elem.tag == "artist":
            return artist

def parse_artist_alias(ctx):
    for event, elem in ctx:
        if event == "end" and elem.tag == "name":
            return {
                "id": elem.attrib["id"],
                "name": elem.text
            }
  
def parse_master(ctx):
    master={}
    for event, elem in ctx:
        if event == "start":
            match elem.tag:
                case "main_release":
                    master["main_release"] = elem.text
                case "artists":
                    master["artists"] = [artist for artist in parse_node_type(ctx, "artist", "artists", parse_release_artist)]
                case "genres":
                    master["genres"] = [genre for genre in parse_node_type(ctx, "genre", "genres", partial(parse_node_text, "genre"))]
                case "styles":
                    master["styles"] =  [style for style in parse_node_type(ctx, "style", "styles", partial(parse_node_text, "style"))]
                case "year":
                    master["year"] = elem.text
                case "title":
                    master["title"] = elem.text
                case _:
                    skip_tag(ctx, elem.tag)
        elif event == "end" and elem.tag == "master":
            master["id"] = elem.attrib["id"]
            return master


def parse_release(ctx):
    release={}
    for event, elem in ctx:
        if event == "start":
            match elem.tag:
                case "artists":
                    release["artists"] = [artist for artist in parse_node_type(ctx, "artist", "artists", parse_release_artist)]
                case "labels":
                    release["labels"] = [label for label in parse_node_type(ctx, "label", "labels", parse_release_label)]
                case "extraartists":
                    release["extraartists"] = [artist for artist in parse_node_type(ctx, "artist", "extraartists", parse_release_artist)]
                case "genres":
                    release["genres"] = [genre for genre in parse_node_type(ctx, "genre", "genres", partial(parse_node_text, "genre"))]
                case "styles":
                    release["styles"] =  [style for style in parse_node_type(ctx, "style", "styles", partial(parse_node_text, "style"))]
                case "tracklist":
                    release["tracklist"] = [track for track in parse_node_type(ctx, "track", "tracklist", parse_release_track)]
                case "country":
                    release["country"] = elem.text
                case "notes":
                    release["notes"] = elem.text
                case "released":
                    release["released"] = elem.text
                case "title":
                    release["title"] = elem.text
                case "master_id":
                    release["master_id"] = elem.text
                    release["is_main_release"] = elem.get("is_main_release", None)
                case _:
                    skip_tag(ctx, elem.tag)
        elif event == "end" and elem.tag == "release":
            return release

def parse_release_artist(ctx):
    artist={}
    for event, elem in ctx:
        if event == "start":
            match elem.tag:
                case "id":
                    artist["id"] = elem.text
                case "anv":
                    artist["anv"] = elem.text
                case "name":
                    artist["name"] = elem.text
                case "role":
                    artist["role"] = elem.text
                case _:
                    skip_tag(ctx, elem.tag)
        elif event == "end" and elem.tag == "artist":
            return artist

def parse_release_label(ctx):
    label={}
    for event, elem in ctx:
        if event == "start":
            label = elem.attrib
        elif event == "end" and elem.tag == "label":
            return label

def parse_release_track(ctx):
    track={}
    for event, elem in ctx:
        if event == "start":
            match elem.tag:
                case "position":
                    track["position"] = elem.text
                case "title":
                    track["title"] = elem.text
                case "duration":
                    track["duration"] = elem.text
                case _:
                    skip_tag(ctx, elem.tag)
        elif event == "end" and elem.tag == "track":
            return track

def parse_label(ctx):
    label={}
    for event, elem in ctx:
        if event == "start":
            match elem.tag:
                case "id":
                    label["id"] = elem.text
                case "name":
                    label["name"] = elem.text
                case "sublabels":
                    label["sublabels"] = label["labels"] = [label for label in parse_node_type(ctx, "label", "sublabels", parse_sublabel)]
                case _:
                    skip_tag(ctx, elem.tag)
        elif event == "end" and elem.tag == "label":
            return label

def parse_sublabel(ctx):
    label={}
    for event, elem in ctx:
        if event == "end" and elem.tag == "label":
            return {
                "id": elem.attrib["id"],
                "label": elem.text
            }

def stream_xml(url, parser):
    s = requests.Session()
    with s.get(url, headers=None, stream=True) as res:
        res.raise_for_status()
        with gzip.GzipFile(fileobj=res.raw, mode='rb') as unzipped:
            ctx = ElementTree.iterparse(unzipped, events=("start", "end"))
            _, root = next(ctx)
            for elem in parser(ctx):
                yield elem
                root.clear()

def stream_xml_file(path, parser):
    f = open(path, "rb")
    with gzip.GzipFile(fileobj=f, mode='rb') as unzipped:
        ctx = ElementTree.iterparse(unzipped, events=("start", "end"))
        _, root = next(ctx)
        for elem in parser(ctx):
            yield elem
            root.clear()

#for elem in stream_xml("https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_20240701_artists.xml.gz", parse_artists_stream):
#    pprint(elem)

#for elem in stream_xml("https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_20240701_labels.xml.gz", parse_labels_stream):
#    pprint(elem)

#for elem in stream_xml("https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_20240701_masters.xml.gz", parse_masters_stream):
#    pprint(elem)

#for elem in stream_xml("https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_20240701_releases.xml.gz", parse_releases_stream):
#    pprint(elem)

step=10000
count=0
last=time.time()*1000
#for elem in stream_xml_file("./discogs_20141201_releases.xml.gz", parse_releases_stream):
for elem in stream_xml("https://discogs-data-dumps.s3-us-west-2.amazonaws.com/data/2024/discogs_20240701_releases.xml.gz", parse_releases_stream):
    #pprint(elem)
    count += 1
    if (count % step) == 0:
        #pprint(elem)
        now=time.time()*1000
        print("> %d events in %fms" % (step, (now - last)))
        last=now

print("read %s releases\n" % count)
