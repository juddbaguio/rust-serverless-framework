.PHONY: build
build:
	cargo lambda build --release -o Zip

deploy: build
	sls deploy --aws-profile ${AWS_PROFILE}

remove:
	sls remove --aws-profile ${AWS_PROFILE}