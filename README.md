# Overview
This is a basic HTTP file server that aims to make retrieving CA certificates a bit easier.
The server will serve **any** file within the `/usr/src/cert-fileserver/certs/` directory. 

# Build
To build, while within the root of the repository, run `docker build -t <tag_name> . `

# Running
To run:
`docker run -d -p <port>:2002 --name <name> -v <volume mappings for certificates> <image>`

Note that the certificate volume mappings must be of the form `<host file location>:/usr/src/cert-fileserver/certs/<filename>`. If you're mapping in a directory, remember that **every** file will be accessible, so remember not to have any private keys!!