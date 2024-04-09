deploy:
	anchor build && anchor deploy

test:
	anchor test --skip-build --skip-deploy 

PHONY: deploy test