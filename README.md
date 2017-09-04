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
    Finished release [optimized] target(s) in 0.0 secs

number of queries = 1
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 29, nanos: 961088951 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14228 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 50537088, elapsed: Duration { secs: 33, nanos: 157365193 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15745 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 11, nanos: 139794132 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5289 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 8, nanos: 109336066 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3850 }

number of queries = 2
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 65157568, elapsed: Duration { secs: 31, nanos: 54529645 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14746 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 60946144, elapsed: Duration { secs: 33, nanos: 939193301 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16117 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 65655296, elapsed: Duration { secs: 12, nanos: 430558764 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5902 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 2105712, size: 8759789136, r: 65655296, elapsed: Duration { secs: 8, nanos: 334620322 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3957 }

number of queries = 4
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 270791360, elapsed: Duration { secs: 33, nanos: 959219912 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16126 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 262368512, elapsed: Duration { secs: 33, nanos: 516911955 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15916 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 282302384, elapsed: Duration { secs: 11, nanos: 584997894 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5500 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 2105712, size: 8759789136, r: 282302384, elapsed: Duration { secs: 8, nanos: 866394171 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4210 }

number of queries = 8
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 884740416, elapsed: Duration { secs: 41, nanos: 370446565 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 19645 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 867894720, elapsed: Duration { secs: 35, nanos: 672514883 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16940 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 938037520, elapsed: Duration { secs: 12, nanos: 600547422 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5983 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 2105712, size: 8759789136, r: 938037520, elapsed: Duration { secs: 9, nanos: 899756964 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4701 }

number of queries = 16
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1158107776, elapsed: Duration { secs: 46, nanos: 995750865 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 22317 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1116730496, elapsed: Duration { secs: 38, nanos: 24225751 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 18057 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1211404880, elapsed: Duration { secs: 14, nanos: 386593893 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6831 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 2105712, size: 8759789136, r: 1211404880, elapsed: Duration { secs: 12, nanos: 989227844 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6167 }
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
