# slogprint

Slogprint is a small util for reading structured logging messages for example docker logs. The reader reads all lines piped to it and formats them with jq.

## Example

`cat test.txt | ./slogprint`
