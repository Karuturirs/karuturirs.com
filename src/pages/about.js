import React from "react"
import Layout from "../components/layout"
import Header from "../components/header"

export default function About() {
  return (
    <div >
      <Header></Header>
      <Layout>
       <div style={{ margin: `3rem auto`, maxWidth: 800, padding: `0 1rem` }}>
          <h1>About me</h1>
          <p>
            I’m good enough, I’m smart enough, and gosh darn it, people like me!
          </p>
        </div>
      </Layout>
    </div>
  )
}