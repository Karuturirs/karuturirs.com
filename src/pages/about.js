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
          <h3>About me</h3>
          <p>
            A developer, who loves developing new stuff and make things easy and some time complex.
            I am so passionate in design and implementation of new products. Having around 10years of experience 
            in Backend development, Bigdata, Security, Content Management. 
          </p>
          
          <p>
            Hobbies are Reading books, Yoga, Board games (Catan being my favourite of all). 
          </p>

          <p>
            Currently started reading The Memory Book.
            I capture all the best information of the books that I read, If you donot have time to read all the book. 
            Just read these outlines which will give you an good idea of that book at <a href="https://github.com/Karuturirs/MyBookMyWords/blob/master/src/ReadBooks-cheats.md">mybookmywords</a>.
          </p>

          <p>
            Hope my knowledge help some one some day.
          </p>

          <p>I'd love to talk! Email me for a virtual coffee chat.</p>
          <p>
            <a href="mailto:hello@karuturirs.com">hello@karuturirs.com</a>
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