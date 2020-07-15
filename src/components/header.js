import React from "react"
import config from "../../data/appConfig"
import { Link } from "gatsby"
import styles from "../components/css/header.css"

const ListLink = props => (
    <li className='menuitems mouse'>
        <Link to={props.to}>{props.children}</Link>
    </li>

)

export default function Header() {
    return (
        <div style={{ margin: `3rem auto`, maxWidth: 900, padding: `0 1rem` }}>
          <header style={{ marginBottom: `1.5rem`, display: `inline`}}>
            <Link to="/" >
                <h3  className="mouse" style={{ display: `inline`, padding: `10px` }}><span role="img" aria-label='logo'>👨‍💻</span>{config.siteTitle}</h3>
            </Link>
            <ul style={{ listStyle: `none`, float: `right`}}>
                <ListLink to="/articles/"  >Articles</ListLink>
                <ListLink to="/contactme/" >Contact Me</ListLink>
            </ul>
          </header>
        </div>
    )
}
