web:
	echo "Building web";
	trunk build;
	cd dist; php -S "localhost:6677";

publish:
	echo "Publishing on proj311.ceyraud.com";
	trunk build;
	scp -r ./dist always:/home/miti/proj311/;
	echo "http://proj311.ceyraud.com";
