# Intro
A CI friendly JSON REST API testing framework, configured through a single JSON file and run with a single binary

# Docs
Running trest is very easy, all you need is a single command:  
```
$ wget bin.tre.st/latest && chmod +x latest && ./latest config.json 
```
You just need to change the `config.json` to the path of your coniguration file.

The configuration file is a JSON file conisting of three keys at its root: `configs`, `tasks` and `tests`.  

The `configs` key contains an array of objects. Each object represents a configuration. The idea behind this is that you can easily run your tests with different configurations. This might be useful if your application supports multiple databases and you want to make sure that your API behaves the same with each.  
Each object can look like this:
```json
{
	"name": "backend", //name of the config
	"description": "tests for the rest api of the backend", //description of the config
	"setup": { //information about how to setup the environment to run the tests in
		"cmd": "curl -O https://raw.githubusercontent.com/T-x-T/TxTs-Treasury/4b0cb752581eb58f20900bdaccb0caf3f0f6ddf5/docker-compose.yml && docker-compose up -d", //shell command to run 
		"finished_condition": { //conditions to meet to consider the application fully started and ready to run tests on
			"endpoint_reachable": "/api/v1" //string containing an api that must be reachable
		}
	},
	"cleanup": { //information on how to clean up the environment
		"cmd": "docker-compose down && rm docker-compose.yml" //shell command to run 
	},
	"api_hostname": "http://localhost:4000" //base url for the json rest api you want to test
}
```

The `tasks` key contains an object with a key for each task. The name of the key can then be referenced to run the task. With each task you can run a single HTTP request. The body of the response can then be used in the test that ran this task. You can use this to authenticate for example.
The keys value is then again an object itself.  
Each object can look like this:
```json
{
	"endpoint": "/api/v1/login", //path of the endpoint
	"method": "POST", //HTTP method to use
	"body": { //body of the request
		...
	}
}
```

The `tests` key contains an array of objects. Each object represents a chain of tests. Trest will run the setup and cleanup defined in the configuration before and after running each chain of tests. If you dont care about the side effects your tests produce you can run all tests in a single chain for best performance. But if you need to start fresh for certain tests, then you need to put them in their own test chain.
Each object can look like this:
```json
{
	"name": "currency", //name of the test chain
	"defaults": [ //Optional defaults for the individual tests, can be overridden in individual test config 
		"cookies": { //Cookies to include with the request
			"accessToken": "$login_as_admin.accessToken" //Create cookie accessToken using the accessToken key from outcome of the before task login_as_admin 
		},
		"before": [ //tasks to run before the test
			"login_as_admin" //name of task
		],
	]
	"tests": [
		... //objects for the individual tests
	]
},
```
The `tests` key then contains the actual tests.  
Each object can look like this:
```json
{
	"name": "retrieval of all currencies works", //name of the test
	"endpoint": "/api/v1/currencies/all", //path of the endpoint to test
	"method": "GET", //HTTP method to use
	"cookies": { //Cookies to include with the request
		"accessToken": "$login_as_admin.accessToken" //Create cookie accessToken using the accessToken key from outcome of the before task login_as_admin 
	},
	"before": [ //tasks to run before the test
		"login_as_admin" //name of task
	],
	"expected_outcome": { //information to check if the reponse matches what we expect
		"status_code_equals": 200, //matches status code
		"body_equals": [{"id":0,"name":"Euro","minor_in_mayor":100,"symbol":"€"},{"id":1,"name":"USD","minor_in_mayor":100,"symbol":"$"}] //matches body of the reponse
	} 
}
```

A full example configuration can be found [here](https://github.com/T-x-T/trest/blob/main/test/sample.json).
