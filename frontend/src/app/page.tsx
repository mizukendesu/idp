export default async function Home() {
  const books = (await (
    await fetch('http://backend:8000/api/v1/books')
  ).json()) as {
    id: number
    title: string
    author: string
    created_at: string
    updated_at: string
  }[]

  return (
    <main className="container mx-auto">
      <h1>Books</h1>
      <ul>
        {books.map((book) => (
          <li key={book.id}>{book.title}</li>
        ))}
      </ul>
    </main>
  )
}
