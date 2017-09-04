# Rust JSON Parser Benchmark

## Download and Generate JSON Data

```
$ mkdir work
$ cd work
$ curl -O http://jsonstudio.com/wp-content/uploads/2014/02/companies.zip
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed
100 14.7M  100 14.7M    0     0  2610k      0  0:00:05  0:00:05 --:--:-- 3428k

$ unzip companies.zip
Archive:  companies.zip
  inflating: companies.json

$ for i in {1..14}; do cat companies.json >> companies_1g.json; done
$ for i in {1..2}; do cat companies_1g.json >> companies_2g.json; done
$ for i in {1..2}; do cat companies_2g.json >> companies_4g.json; done
$ for i in {1..2}; do cat companies_4g.json >> companies_8g.json; done
```

## Run Benchmark

```
$ ./bench.sh
build
   Compiling pikkr v0.9.0
   Compiling rjpb v0.1.0 (file:///Users/JP21605/dev/rust-json-parser-benchmark)
    Finished release [optimized] target(s) in 7.26 secs

number of queries = 1
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 28, nanos: 340447243 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 13458 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 50537088, elapsed: Duration { secs: 30, nanos: 422063856 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14446 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 10, nanos: 438301403 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4956 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 7, nanos: 558332908 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3589 }

number of queries = 2
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 65157568, elapsed: Duration { secs: 29, nanos: 371349672 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 13948 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 60946144, elapsed: Duration { secs: 31, nanos: 439334465 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14929 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 65655296, elapsed: Duration { secs: 10, nanos: 624603285 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5044 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 2105712, size: 8759789136, r: 65655296, elapsed: Duration { secs: 7, nanos: 821776504 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3714 }

number of queries = 4
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 270791360, elapsed: Duration { secs: 32, nanos: 253442165 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15316 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 262368512, elapsed: Duration { secs: 31, nanos: 778681344 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15090 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 282302384, elapsed: Duration { secs: 10, nanos: 974000200 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5210 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 2105712, size: 8759789136, r: 282302384, elapsed: Duration { secs: 8, nanos: 213547820 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3900 }

number of queries = 8
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 884740416, elapsed: Duration { secs: 39, nanos: 879395806 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18938 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 867894720, elapsed: Duration { secs: 33, nanos: 726012891 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16015 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 938037520, elapsed: Duration { secs: 11, nanos: 451796561 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5437 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 2105712, size: 8759789136, r: 938037520, elapsed: Duration { secs: 8, nanos: 919990122 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4235 }

number of queries = 16
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1158107776, elapsed: Duration { secs: 45, nanos: 77639732 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 21406 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1116730496, elapsed: Duration { secs: 36, nanos: 161543653 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 17172 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1211404880, elapsed: Duration { secs: 12, nanos: 621299696 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5993 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 2105712, size: 8759789136, r: 1211404880, elapsed: Duration { secs: 10, nanos: 875859537 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5163 }
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
