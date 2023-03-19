# Wasm Server v2

- Escrever um api que armazena e executa arquivos `.wasm`
- Usar queues/channels para receber os arquivos `.wasm`
- Usar tokio + hyper

A ideia aqui é escrever um CRUD básico no padrão REST API.

1. A aplicação inicia
2. então 1 runners é criados
3. então 1 worker é criado
4. então o servidor é iniciado e espera por requests
5. o server recebe a request e envia para o `handler_router`
6. o `handler_router` analiza o tipo de request que pode ser
   1. GET /wasm/{id} - precisa retornar um o wasm referente ao id
   2. POST /wasm - precisa adicionar um novo `.wasm`
   3. PUT /wasm/{id} - precisa atualizar algum dado de um `.wasm` existente
   4. DELETE /wasm/{id} - precisa apagar um `.wasm` existente
   5. POST /wasm/{id} - precisa executar o `.wasm` existente
7. caso seja GET o handler vai ler do banco e devolver o WasmStorage
    - WasmStorage é um struct que contem
      - wasm: Vec<u8>
      - meta_data: struct WasmMetaData:
        - run_function: String,
        - size: usize,
        - param: Vec<WasmParam>:
            - WasmParam<T>:
                name: String,
                param_type: String
7. se a request não for o GET ou um POST /wasm/{id} então
8. então o handler encapsula a request para um WasmRequest
    - WasmRequest é um struct que contem
      - wasm: Vec<u8>
      - funtion: String
9. depois o handler encapsula o WasmRequest em um WasmJob
    - WasmJob é um struct que contem
      - wasm: Vec<u8>
      - meta_data: struct JobMetaData:
        - id: String,
        - run_function: String,
        - size: usize,
        - job_type: enum JobType:
            - SAVE
            - MODIFY
            - DELETE
10. então a function coloca o WasmRequest no channel tx
12. O runner olha o channel rx
    1. pega o wasm
    2. executa a ação de acordo com o req_type do WasmRequest

![](./diagram.png)

# ~~Library~~ Wasm Server v2

- Agora ao inves de `wasm` o server deve salvar um `.wasm` como um struct `Wasm`
- Os arquivos devem ser salvos em disco (DB)
- O DB deve ser embarcado com a aplicação
- O server deve poder executar o código desse arquivo `.wasm` salvo.
