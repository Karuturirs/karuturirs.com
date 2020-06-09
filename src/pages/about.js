import React from "react"
import Layout from "../components/layout"
import Header from "../components/header"
import { graphql } from "gatsby"

export default function About({ data }) {
  return (
    <div >
      <Header></Header>
      <Layout>
          <h3>About {data.site.siteMetadata.title}</h3>
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