import React from "react"
import config from "../../data/appConfig"

export default function Header() {
    return (
        <div style={{ margin: `3rem auto`, maxWidth: 650, padding: `0 1rem` }}>
            <a href="#default" className="logo">{`${config.siteTitle}`}</a>
            <div className="header-right">
                <a className="active" href="#home">Home</a>
                <a href="#contact">Contact</a>
                <a href="#about">About</a>
            </div>
        </div> 
    )
}