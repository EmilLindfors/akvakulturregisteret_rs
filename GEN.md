```
scoop install openapi-generator-cli
scoop bucket add java
scoop install openjdk
```

```
openapi-generator-cli generate -g rust -o C:\Users\emill\Desktop\dbt\akvakulturregisteret_rs\temp -i C:\Users\emill\Desktop\dbt\akvakulturregisteret_rs\spec.json --additional-properties=library=reqwest-trait,mockall=true,packageName=akvakulturregisteret_rs,supportMultipleResponses=true,topLevelApiClient=true,useBonBuilder=true
```
