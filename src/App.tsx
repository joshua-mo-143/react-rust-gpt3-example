import React from 'react'

function App() {
  const fetchData = async () => {
    const res = await fetch(`/api/prompt`);
    const data = await res.json()
    console.log(data)
    return data;
  }

  const setText = (data: string) => {
    let text = document.querySelector('#prompt-text') as HTMLParagraphElement;

        text.innerText = data;
  }

  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault()
    try {
      fetchData().then(data => {
        setText(data.split("\n")[0])
      })
    } catch (err: any) {
      console.log(`Error: ${err}`);
      setText(err)
    }
  }

  return (
    <div className="w-full h-full flex flex-col align-center items-center mt-10 gap-4">
      <p className='text-2xl'>Name Generator</p>
      <form onSubmit={handleSubmit}>
        <button type="submit" className='bg-gray-300 px-5 py-2 shadow-md'>Generate a name!</button>
      </form>
      <div id="prompt-container" className='text-sm p-5 bg-stone-100 shadow-md w-[calc(100%-5rem)]'>
        <p id="prompt-text">Try generating a prompt!</p>
      </div>
    </div>
  )
}

export default App
