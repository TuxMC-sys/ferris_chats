# Ferris Chat's Server
This binary is the server for Ferris Chat and can be used standalone. 
## Host
By default, this server is hosted on 0.0.0.0 on port 3000. A configuration file 
system will be added at some point. 
## Security
This is unsecured and uses http. WIP.
## GET Requests
How you retrieve messages.
### Routes
The following routes are provided for `GET` requests:
- `/{first}/{amount} `\
This gets `amount` messages after the `first` specified message. If the request is non-numeric, negative, or includes 
messages that don't exist, it will return a 400 Error
- `/time/{time}`\
This will get all messages received after `time`. The variable `time` should be in the format prescribed by RFC3339, which
 should look something like "2012-12-12T12:12:12Z". If the format is wrong, you'll receive an error 400, otherwise, if the requested 
messages don't exist, you'll receive an error 404.
- `/all`\
This will return all messages on the server and cannot return an error, though can
return an object with just an empty array inside.
- `/count`\
This will return an integer representing the number of messages on the server.
### Format
For endpoints except for `/count`, you will receive the a JSON object in the following format:
```
{"messages": [*message objects*]}
```
If there are message objects, they will be in the following format, seperated by commas:
```
{"content": "Message body",
"author": "'Unknown' or the author.",
"time": "Time message was recieved in RFC3339 format}, 
```
## POST Requests
How you send messages
### Endpoints
The following endpoint is provided for `POST` requests:\
- `\endpoint`\
Receives incoming message objects. Takes `Content-Type` of `application/json`.
### Format
A received message must contain at least the following: 
```
{"content": "Message Body"} 
```
and should have a body as follows:
```
{"content": "Message Body", "author": "Author"}
```
## Other
This is a work in progress. If you find bugs, please let me know. Also, once ran, you need to hit ctrl+c to exit and save. 
Messages are saved in `$PATH/.ferris_chats/messages.json`.