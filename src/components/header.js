import React from "react"
import config from "../../data/appConfig"

export default function Header() {
    return (
        <div class="header">
            <a href="#default" class="logo">{`${config.siteTitle}`}</a>
            <div class="header-right">
                <a class="active" href="#home">Home</a>
                <a href="#contact">Contact</a>
                <a href="#about">About</a>
            </div>
        </div> 
    )
}