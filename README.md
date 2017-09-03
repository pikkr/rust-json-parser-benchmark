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
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 29, nanos: 422812269 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 13972 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 50537088, elapsed: Duration { secs: 28, nanos: 64140645 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 13327 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 10, nanos: 650018302 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5056 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid print: false train_num: 100
num: 2105712, size: 8759789136, r: 54748512, elapsed: Duration { secs: 7, nanos: 838286606 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3722 }

number of queries = 2
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 65157568, elapsed: Duration { secs: 30, nanos: 27010837 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14258 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 60946144, elapsed: Duration { secs: 28, nanos: 147929871 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 13367 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 65655296, elapsed: Duration { secs: 10, nanos: 837647787 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5145 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners print: false train_num: 100
num: 2105712, size: 8759789136, r: 65655296, elapsed: Duration { secs: 8, nanos: 70993138 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 3832 }

number of queries = 4
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 270791360, elapsed: Duration { secs: 33, nanos: 794537563 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 16048 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 262368512, elapsed: Duration { secs: 29, nanos: 104457317 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 13821 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 282302384, elapsed: Duration { secs: 11, nanos: 325916447 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5377 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links print: false train_num: 100
num: 2105712, size: 8759789136, r: 282302384, elapsed: Duration { secs: 8, nanos: 724719190 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4143 }

number of queries = 8
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 884740416, elapsed: Duration { secs: 41, nanos: 241916454 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 19584 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 867894720, elapsed: Duration { secs: 31, nanos: 485856300 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 14951 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 938037520, elapsed: Duration { secs: 12, nanos: 274527987 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 5828 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds print: false train_num: 100
num: 2105712, size: 8759789136, r: 938037520, elapsed: Duration { secs: 10, nanos: 68682789 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 4780 }

number of queries = 16
file_path: work/companies_8g.json, parser_name: serde_json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1158107776, elapsed: Duration { secs: 47, nanos: 602202634 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 22605 }
file_path: work/companies_8g.json, parser_name: json, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1116730496, elapsed: Duration { secs: 33, nanos: 608407159 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 15959 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 1000000000000000000
num: 2105712, size: 8759789136, r: 1211404880, elapsed: Duration { secs: 14, nanos: 499979198 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6885 }
file_path: work/companies_8g.json, parser_name: pikkr, queries: $._id.$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address print: false train_num: 100
num: 2105712, size: 8759789136, r: 1211404880, elapsed: Duration { secs: 13, nanos: 682013791 }, average size: 4160, average elapsed: Duration { secs: 0, nanos: 6496 }
```

## Restrictions

* [Rust nightly channel](https://github.com/rust-lang-nursery/rustup.rs/blob/master/README.md#working-with-nightly-rust) and [CPUs with AVX2](https://en.wikipedia.org/wiki/Advanced_Vector_Extensions#CPUs_with_AVX2) are needed to build Rust source code which depends on Pikkr and run the executable binary file because Pikkr uses AVX2 Instructions.

## Contributing

Any kind of contribution (e.g. comment, suggestion, question, bug report and pull request) is welcome.
