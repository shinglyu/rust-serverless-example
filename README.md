# Installation
* Install rustup: https://rustup.rs/

        % rustup show
        Default host: x86_64-apple-darwin

        stable-x86_64-apple-darwin (default)
        rustc 1.31.1 (b6c32da9b 2018-12-18)

* MacOS
  * `rustup target add x86_64-unknown-linux-musl`
  * `brew install filosottile/musl-cross/musl-cross`
  * Configure the linker in `./cargo/config`
  * `ln -s /usr/local/bin/x86_64-linux-musl-gcc /usr/local/bin/musl-gcc`

# Test with AWS Console
* Go to Services > Lambda > Create function
* Author from scratch
  * Function name: test-rust-lambda
  * Runtime: Custom runtime
  * Permissions: Create a new role with basic Lambda permissions
* In the lambda configuration page:
  * Function code > Code entry type: Upload a .zip file
  * Zip the `./target/x86_64-unknown-linux-musl/release/bootstrap` file into a `package.zip`
  * Upload the `package.zip`
* Click the "Test" button
  * Create new test event
  * In the payload type `{ "firstName": "Foo" }`
  * Save
  * Click "Test"

* Create an API gateway to test:
  * Go to API Gateway 
  * Create API
    * protocol: REST
    * Create new API: New API
    * Settings:
      * API name: test-rust-api
      * Endpoint Type: Regional
  * Under the Resouces tab
    * Actions > Create Method > POST
    * Integration type: Lambda function
    * Lambda Function: test-rust-lambda
    * Save
  * Select the newly create PSOT method > TEST   
    * In the Request body: `{ "firstName": "Foo" }`
  * Once it's working, let's deploy it on the internet: 
    * Actions > Deploy API
    * Deployment stage > [New Stage]
      * Stage name: dev
    * Deploy
  * Copy the new URL
    * Test it with Postman or cURL

        curl -X POST \
          https://xxxxxxxx.execute-api.eu-central-1.amazonaws.com/dev \
          -H 'Content-Type: application/json' \
          -d '{"firstName": "Shing"}'

# References
* https://aws.amazon.com/blogs/opensource/rust-runtime-for-aws-lambda/

