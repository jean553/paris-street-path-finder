# paris-street-path-finder

An attempt to make a program able to find the best path from a departure to an arrival destination using Paris open data.

## Build the container

```sh
vagrant up
```

## Connect into the container

```sh
vagrant ssh
```

## Extract data from Paris Open Data files

### Streets polygons

Download streets polygons full CSV file from [here](https://opendata.paris.fr/explore/dataset/plan-de-voirie-chaussees/download/?format=csv&timezone=Europe/Berlin&lang=fr&use_labels_for_header=true&csv_separator=%3B).

Extract simple polygons of every streets as JSON arrays:

```sh
cat plan-de-voirie-chaussees.csv | grep -v 'MultiPolygon' | grep -v 'geo_shape' | cut -d';' -f7 | sed 's/.*\[\[\[/\[/' | sed 's/\]\]//' | sed 's/}"//g' | sed 's/^/\[/g' | sed 's/$/\]/g' > data/streets_polygons
```

## Credits

Use the following open data from Paris Open data, distributed under [ODbL licence](https://opendatacommons.org/licenses/odbl/summary/).

 * [streets](https://opendata.paris.fr/explore/dataset/plan-de-voirie-chaussees)
 * [walking areas](https://opendata.paris.fr/explore/dataset/aires-pietonnes)
 * [addresses](https://opendata.paris.fr/explore/dataset/adresse_paris)
