import React from 'react'
import reactLogo from './assets/react.svg'

function App() {
  const [prompt, setPrompt] = React.useState<string>("");

  const fetchData = async () => {
    const res = await fetch(`/api/prompt`);
    const data = await res.json()
    return data;
  }

  const setText = () => {
    let text = document.querySelector('#prompt-text') as HTMLParagraphElement;

        text.innerText = prompt;
  }

  const handleSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault()
    try {
      fetchData().then(data => {
        setPrompt(data);
        setText()
      })
    } catch (err: any) {
      console.log(`Error: ${err}`);
      setPrompt(`Error: ${err}`);
      setText()
    }
  }

  return (
    <div className="w-full h-full flex flex-col align-center items-center mt-10 gap-4">
      <p className='text-2xl'>Design Brief Generator</p>
      <form onSubmit={handleSubmit}>
        <button type="submit" className='bg-gray-300 px-5 py-2 shadow-md'>Generate a Creative Brief</button>
      </form>
      <div id="prompt-container" className='text-sm p-5 bg-stone-100 shadow-md w-[calc(100%-5rem)]'>
        <p id="prompt-text">{prompt ? "H": "Try generating a prompt!"}</p>
      </div>
    </div>
  )
}

export default App
