import asyncio
import requests
import aiohttp
import time

async def send_requests():
    url = 'http://127.0.0.1:3000/book'
    headers = {'Content-Type': 'application/json'}
    total_requests = 10000
    success_requests = 0

    async with aiohttp.ClientSession(headers=headers) as session:
        start_time = time.time()
        for i in range(total_requests):
            async with session.post(url, json={"title": "Grazy", "year": i}) as response:
                if response.status != 200:
                    print(f"🚫 Erro ao criar livro {i+1}")
                else:
                    success_requests += 1
        end_time = time.time()
        time_elapsed = end_time - start_time
        requests_per_second = success_requests / time_elapsed
        print(f"✅ {success_requests} livros foram criados com sucesso!")
        print(f"🕒 Tempo total de execução: {time_elapsed:.2f} segundos")
        print(f"🚀 Taxa de requests por segundo: {requests_per_second:.2f}")
        
        # Verificar se todos os livros foram salvos
        all_books = requests.get(url).json()
        num_books_created = len(all_books)
        if num_books_created == total_requests:
            print("📚 Todos os livros foram salvos!")
        else:
            num_books_missing = total_requests - num_books_created
            print(f"❗️ {num_books_missing} livros não foram salvos.")
        
# Cria um loop de eventos e executa a função send_requests
loop = asyncio.get_event_loop()
loop.run_until_complete(send_requests())