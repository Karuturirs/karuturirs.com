import React from "react"
import Layout from "../components/layout"
import Header from "../components/header"
import { graphql } from "gatsby"
import SEO from "../components/seo"

export default function About() {
  return (
    <div >
      <SEO title="About me" description="Software engineer looking for cool stuff to develop." /> 
      <Header></Header>
      <Layout>
          <h3>Articles</h3>
          <p>
            I’m good enough, I’m smart enough, and gosh darn it, people like me!
          </p>
      </Layout>
    </div>
  )
}

export const query = graphql`
  query {
    site {
       siteMetadata {
         title
       }
    }
  }
`