#!/bin/bash -eu

FILE=work/companies_1g.json
PRINT=false
TRAIN_NUM=100

function bench() {
	queries=$1
	./target/release/rjpb $FILE serde_json $queries $PRINT
	./target/release/rjpb $FILE json       $queries $PRINT
	./target/release/rjpb $FILE pikkr      $queries $PRINT
	./target/release/rjpb $FILE pikkr      $queries $PRINT $TRAIN_NUM
}

echo "build"
RUSTFLAGS="-C target-cpu=native" cargo build --release

echo ""
echo "number of queries = 1"
bench "$._id.\$oid"

echo ""
echo "number of queries = 2"
bench "$._id.\$oid,$.partners"

echo ""
echo "number of queries = 4"
bench "$._id.\$oid,$.partners,$.name,$.external_links"

echo ""
echo "number of queries = 8"
bench "$._id.\$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds"

echo ""
echo "number of queries = 16"
bench "$._id.\$oid,$.partners,$.name,$.external_links,$.permalink,$.screenshots,$.crunchbase_url,$.video_embeds,$.homepage_url,$.blog_url,$.blog_feed_url,$.twitter_username,$.category_code,$.number_of_employees,$.tag_list,$.email_address"
